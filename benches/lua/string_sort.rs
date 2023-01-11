use std::sync::Arc;

use bevy::{
    prelude::{ReflectResource, Resource},
    reflect::Reflect,
};
use bevy_mod_scripting::lua::tealr::mlu::mlua::{Lua, MetaMethod, UserData, UserDataMethods};

use bevy_script_api::{impl_lua_newtype, impl_script_newtype};
use criterion::{criterion_group, criterion_main, Criterion};

#[derive(Resource, Default, Reflect, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[reflect(Resource)]
struct RustData {
    string: String,
}

impl RustData {
    pub fn new(str: String) -> Self {
        return Self { string: str };
    }
}

impl_script_newtype!(
    #[languages(lua)]
    RustData: Debug
        + Clone
        + Fields(string: Raw(String))
        + Methods(
            new(Raw(String)) -> self
        )
        lua impl {
            (MetaMethod::Lt) => |_,s : &LuaRustData,o : LuaRustData| {
                s.val(|s| s < o.inner()?).into()
            }
        }

);

impl UserData for RustData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("new", |_, s: String| Ok(RustData(Arc::new(s))));
        methods.add_meta_method(MetaMethod::Lt, |_, this, rhs: Self| Ok(this < &rhs));
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.0.to_string()));
    }
}

fn benchmark(c: &mut Criterion) {
    let lua = Lua::new();
    let globals = lua.globals();
    globals
        .set("RustData", lua.create_proxy::<RustData>().unwrap())
        .unwrap();
    globals
        .set(
            "rand",
            lua.create_function(|_, n: u32| Ok(rand::random::<u32>() % n))
                .unwrap(),
        )
        .unwrap();
    #[cfg(feature = "mlua_luau")]
    lua.sandbox(true).unwrap();
    let f = lua
        .load(include_str!("sort_userdata.lua"))
        .into_function()
        .unwrap();
    c.bench_function("Sort userdata", |b| {
        b.iter(|| {
            f.call::<_, ()>(()).unwrap();
        });
    });
}
criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark,
}
criterion_main!(benches);
