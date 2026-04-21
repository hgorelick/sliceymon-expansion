//! Shared fight unit emission — emits FightUnit structs to textmod format.
//!
//! Used by boss_emitter and (in future) phase emitters for any fight-containing context.

use crate::ir::FightUnit;

/// Emit a single fight unit's properties.
pub fn emit_fight_unit(out: &mut String, unit: &FightUnit) {
    if unit.head_paren {
        // Head-paren shapes (mutually exclusive):
        //   Case A — `(Template.n.Name).rest`: name sits inside the head paren.
        //            Body order omits `Name`; the rest (chain, hp, sd, img, …) emits after.
        //   Case B — `(Template.((nested))).rest`: nested group sits inside head paren.
        //            Body order omits `Nested`; name emits as normal body prop.
        let name_in_head = !unit.body_order.iter().any(|m|
            matches!(m, crate::ir::FightUnitMarker::Name)
        ) && !unit.name.is_empty() && unit.name != unit.template;
        let nested_in_head = unit.nested_units.is_some()
            && !unit.body_order.iter().any(|m|
                matches!(m, crate::ir::FightUnitMarker::Nested)
            );

        out.push('(');
        out.push_str(&unit.template);
        if name_in_head {
            out.push_str(".n.");
            out.push_str(&unit.name);
        } else if nested_in_head {
            if let Some(ref nested) = unit.nested_units {
                if unit.nested_single_paren {
                    out.push_str(".(");
                    for (i, child) in nested.iter().enumerate() {
                        if i > 0 { out.push('+'); }
                        emit_fight_unit(out, child);
                    }
                    out.push(')');
                } else {
                    out.push_str(".((");
                    for (i, child) in nested.iter().enumerate() {
                        if i > 0 { out.push('+'); }
                        emit_fight_unit(out, child);
                    }
                    out.push_str("))");
                }
            }
        }
        out.push(')');
        emit_body_props(out, unit, name_in_head);
        return;
    }
    out.push_str(&unit.template);
    emit_body_props(out, unit, false);
}

fn emit_body_props(out: &mut String, unit: &FightUnit, name_inside_head: bool) {
    use crate::ir::FightUnitMarker as M;

    // Pick the driving order: source body_order (preserves source layout) or
    // canonical fallback when body_order is empty (new units).
    let order: Vec<M> = if !unit.body_order.is_empty() {
        unit.body_order.clone()
    } else {
        let mut ord = Vec::new();
        if unit.nested_units.is_some() { ord.push(M::Nested); }
        if unit.color.is_some() { ord.push(M::Col); }
        if unit.hsv.is_some() { ord.push(M::Hsv); }
        if let Some(ref chain) = unit.modifier_chain {
            for idx in 0..chain.segments.len() {
                ord.push(M::Chain(idx));
            }
        }
        if unit.hp.is_some() { ord.push(M::Hp); }
        if unit.sd.is_some() { ord.push(M::Sd); }
        if !name_inside_head { ord.push(M::Name); }
        if unit.doc.is_some() { ord.push(M::Doc); }
        if unit.sprite.is_some() { ord.push(M::Img); }
        if unit.template_override.is_some() { ord.push(M::TemplateOverride); }
        if unit.part.is_some() { ord.push(M::Part); }
        ord
    };

    for marker in &order {
        match marker {
            M::Nested => {
                if let Some(ref nested) = unit.nested_units {
                    if unit.nested_single_paren {
                        out.push_str(".(");
                        for (i, child) in nested.iter().enumerate() {
                            if i > 0 { out.push('+'); }
                            emit_fight_unit(out, child);
                        }
                        out.push(')');
                    } else {
                        out.push_str(".((");
                        for (i, child) in nested.iter().enumerate() {
                            if i > 0 { out.push('+'); }
                            emit_fight_unit(out, child);
                        }
                        out.push_str("))");
                    }
                }
            }
            M::Col => {
                if let Some(c) = unit.color {
                    out.push_str(".col.");
                    out.push(c);
                }
            }
            M::Hsv => {
                if let Some(ref hsv) = unit.hsv {
                    out.push_str(".hsv.");
                    out.push_str(hsv);
                }
            }
            M::Chain(idx) => {
                if let Some(ref chain) = unit.modifier_chain {
                    if let Some(seg) = chain.segments.get(*idx) {
                        match seg {
                            crate::ir::ChainSegment::Item { sub_entries } => {
                                out.push_str(".i.");
                                out.push_str(&crate::builder::chain_emitter::emit_chain_entries(sub_entries));
                            }
                            crate::ir::ChainSegment::Sticker { sub_entries } => {
                                out.push_str(".sticker.");
                                out.push_str(&crate::builder::chain_emitter::emit_chain_entries(sub_entries));
                            }
                        }
                    }
                }
            }
            M::Hp => {
                if let Some(hp) = unit.hp {
                    out.push_str(".hp.");
                    out.push_str(&hp.to_string());
                }
            }
            M::Sd => {
                if let Some(ref sd) = unit.sd {
                    out.push_str(".sd.");
                    out.push_str(&sd.emit());
                }
            }
            M::Name => {
                if !name_inside_head {
                    out.push_str(".n.");
                    out.push_str(&unit.name);
                }
            }
            M::Doc => {
                if let Some(ref doc) = unit.doc {
                    out.push_str(".doc.");
                    out.push_str(doc);
                }
            }
            M::Img => {
                if let Some(ref s) = unit.sprite {
                    if !s.img_data().is_empty() {
                        out.push_str(".img.");
                        out.push_str(s.img_data());
                    }
                }
            }
            M::TemplateOverride => {
                if let Some(ref t) = unit.template_override {
                    out.push_str(".t.");
                    out.push_str(t);
                    // Post-override keywords emit immediately after the override,
                    // matching source: `.t.jinx.allitem.k.wither`.
                    for kw in &unit.post_override_keywords {
                        out.push_str(".k.");
                        out.push_str(kw);
                    }
                }
            }
            M::Part => {
                if let Some(part) = unit.part {
                    out.push_str(".part.");
                    out.push_str(&part.to_string());
                }
            }
        }
    }
}
