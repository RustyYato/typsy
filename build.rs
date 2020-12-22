use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::Write,
    path::Path,
};

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

const MAX_TUPLE_SIZE: u32 = {
    #[cfg(not(feature = "bigger_tuples"))]
    {
        16
    }
    #[cfg(all(not(feature = "extreme_tuples"), feature = "bigger_tuples"))]
    {
        32
    }
    #[cfg(feature = "extreme_tuples")]
    {
        128
    }
};

fn main() -> Result {
    let out_dir = std::env::var_os("OUT_DIR").ok_or("OUT_DIR env var not set")?;
    let out_dir = Path::new(&out_dir);

    let convert_tuple = File::create(out_dir.join("convert_tuple.rs"))?;
    build_convert_tuple(&convert_tuple)?;

    Ok(())
}

struct Vars {
    name: char,
    prefix: &'static dyn Display,
    postfix: &'static dyn Display,
    count: u32,
}

impl Display for Vars {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 1..=self.count {
            write!(f, "{}{}{}{}", self.prefix, self.name, i, self.postfix)?;
        }

        Ok(())
    }
}

fn build_convert_tuple(mut out: &File) -> Result {
    for i in 0..=MAX_TUPLE_SIZE {
        write!(
            out,
            "
            impl<{type_vars}> Convert for ({type_vars}) {{
                type HList = crate::HList!({type_vars});

                fn into_hlist(self) -> Self::HList {{ let ({vars}) = self; crate::hlist!({vars}) }}
                fn from_hlist(crate::hlist_pat!({vars}): Self::HList) -> Self {{ ({vars}) }}
            }}
            ",
            type_vars = Vars {
                count: i,
                name: 'A',
                prefix: &"",
                postfix: &",",
            },
            vars = Vars {
                count: i,
                name: 'a',
                prefix: &"",
                postfix: &",",
            },
        )?
    }
    Ok(())
}
