/** ++ut jets (compiler backend and pretty-printer)
 */
use ares_macros::tas;
use crate::interpreter::{Context, interpret};
use crate::jets::util::*;
use crate::jets::JetErr::*;
use crate::jets::Result;
use crate::noun::{Noun, D, T};

crate::gdb!();

/*
u3_noun
u3wfu_crop(u3_noun cor)
{
  u3_noun bat, sut, ref, van;

  if (  (c3n == u3r_mean(cor, u3x_sam, &ref, u3x_con, &van, 0))
     || (u3_none == (bat = u3r_at(u3x_bat, van)))
     || (u3_none == (sut = u3r_at(u3x_sam, van))) )
  {
    return u3m_bail(c3__fail);
  }
  else {
    u3_weak vet = u3r_at(u3qfu_van_vet, van);
    c3_m  fun_m = 141 + c3__crop + ((!!vet) << 8);
    u3_noun key = u3z_key_3(fun_m, sut, ref, bat);
    u3_weak pro = u3z_find(key);

    if ( u3_none != pro ) {
      u3z(key);
      return pro;
    }
    else {
      pro = u3n_nock_on(u3k(cor), u3k(u3x_at(u3x_bat, cor)));
      return u3z_save(key, pro);
    }
  }
}
*/

pub fn jet_crop(context: &mut Context, subject: Noun) -> Result {
    let ref_ = slot(subject, 6)?;
    let van = slot(subject, 7)?;
    let bat = slot(van , 2)?;
    let sut = slot(van , 6)?; // why does vere bail:fail if any of these are none?

    let vet = slot(van, 59)?;
    let fun = (141 + tas!(b"crop")) + ((vet.as_atom()?.as_direct()?.data() as u64) << 8);
    let mut key = T(context.stack, &[D(fun), sut, ref_, bat]);
    let pro = context.cache.lookup(context.stack, &mut key);

    match pro {
        Some(pro) => Ok(pro),
        None => {
            let res = interpret(context, subject, slot(subject, 2)?);
            match res {
                Err(_) => {
                    Err(Deterministic)
                },
                Ok(pro) => {
                    context.cache.insert(context.stack, &mut key, pro);
                    Ok(pro)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jets::util::test::{assert_jet, assert_jet_ubig, init_stack, A};
    use crate::mem::NockStack;
    use crate::noun::{Noun, D, T};
    use ibig::ubig;

    fn atoms(s: &mut NockStack) -> (Noun, Noun, Noun, Noun, Noun) {
        (atom_0(s), atom_24(s), atom_63(s), atom_96(s), atom_128(s))
    }

    fn atom_0(_stack: &mut NockStack) -> Noun {
        D(0)
    }

    fn atom_24(_stack: &mut NockStack) -> Noun {
        D(0x876543)
    }

    fn atom_63(_stack: &mut NockStack) -> Noun {
        D(0x7fffffffffffffff)
    }

    fn atom_96(stack: &mut NockStack) -> Noun {
        A(stack, &ubig!(0xfaceb00c15deadbeef123456))
    }

    fn atom_128(stack: &mut NockStack) -> Noun {
        A(stack, &ubig!(0xdeadbeef12345678fedcba9876543210))
    }

    #[test]
    fn test_crop() {
        let s = &mut init_stack();
    }
}
