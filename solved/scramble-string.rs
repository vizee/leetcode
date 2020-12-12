impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        use std::collections::hash_map::HashMap;
        struct Context<'a> {
            vis: HashMap<(usize, usize, usize), bool>,
            s1: &'a [u8],
            s2: &'a [u8],
        }
        fn is_sub_scramble(ctx: &mut Context, a: usize, b: usize, c: usize) -> bool {
            if let Some(ok) = ctx.vis.get(&(a, b, c)) {
                return *ok;
            }
            let mut ok = false;
            match c {
                1 => ok = ctx.s1[a] == ctx.s2[b],
                2 => ok = (ctx.s1[a] == ctx.s2[b] && ctx.s1[a + 1] == ctx.s2[b + 1])
                    || (ctx.s1[a] == ctx.s2[b + 1] && ctx.s1[a + 1] == ctx.s2[b]),
                _ => {
                    for n in 1..c {
                        if is_sub_scramble(ctx, a, b, n)
                            && is_sub_scramble(ctx, a + n, b + n, c - n) {
                            ok = true;
                            break;
                        } else if is_sub_scramble(ctx, a, b + c - n, n)
                            && is_sub_scramble(ctx, a + n, b, c - n) {
                            ok = true;
                            break;
                        }
                    }
                }
            }
            ctx.vis.insert((a, b, c), ok);
            ok
        }
        s1.len() == s2.len() && is_sub_scramble(&mut Context {
            vis: HashMap::with_capacity(s1.len() * s1.len()),
            s1: s1.as_bytes(),
            s2: s2.as_bytes(),
        }, 0, 0, s1.len())
    }
}
