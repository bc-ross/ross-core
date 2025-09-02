/// Functions for adding generic semester constraints (e.g., max credits).
use super::context::ModelBuilderContext;
use cp_sat::builder::{IntVar, LinearExpr};

pub fn add_semester_constraints<'a>(ctx: &mut ModelBuilderContext<'a>) {
    // For each semester, create an IntVar for total semester credits and add a constraint
    // Also enforce max credits per semester for semesters >= 1
    ctx.semester_credit_vars = Vec::new();
    let max_domain_upper = ctx.max_credits_per_semester * ctx.courses.len() as i64;
    for s in 0..ctx.num_semesters {
        let weighted_terms: Vec<(i64, _)> = ctx
            .courses
            .iter()
            .enumerate()
            .map(|(i, c)| (c.credits, ctx.vars[i][s]))
            .collect();
        let weighted_sum: LinearExpr = weighted_terms.into_iter().collect();
        // Create IntVar for semester credits
        let domain = vec![(0, max_domain_upper)];
        let credit_var: IntVar = ctx.model.new_int_var(domain.clone());
        // credit_var == weighted_sum
        ctx.model.add_eq(credit_var.clone(), weighted_sum);
        ctx.semester_credit_vars.push(credit_var.clone());
        // Enforce max credits per semester for scheduled semesters (skip semester 0)
        if s >= 1 {
            ctx.model.add_le(credit_var, ctx.max_credits_per_semester);
        }
    }
}
