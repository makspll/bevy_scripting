# Create the benchmark executable with debugging symbols, but do not run it. We
# don't want valgrind to profile the compiler, so we have the "--no-run" flag. We
# also need debugging symbols so valgrind can track down source code
# appropriately. It blows my mind to this day that compiling with optimizations +
# debugging symbols is a thing. For so long I thought they were mutually
# exclusive.
# RUSTFLAGS="-g" cargo bench  --no-run

# Let's say this was the executable
EXEC="./target/release/"

# Now identify a single test that you want profiled. Test identifiers are
# printed in the console output, so I'll use the one that I posted earlier
T_ID="Hello/World"

TIME="10"
# Have valgrind profile criterion running our benchmark for 10 seconds


# # valgrind outputs a callgrind.out.<pid>. We can analyze this with kcachegrind
# kcachegrind

comp_benches:
	RUSTFLAGS="-g" cargo bench --no-run 

valgrind:
	valgrind --tool=callgrind \
			--dump-instr=yes \
			--collect-jumps=yes \
			--simulate-cache=yes \
			${EXEC} --bench  ${T_ID} 
generate_api:
	cd bevy_mod_scripting_derive && \
	cargo run -- \
	--json "../target/doc/bevy_asset.json" \
	--json "../target/doc/bevy_ecs.json" \
	--json "../target/doc/bevy_pbr.json" \
	--json "../target/doc/bevy_render.json" \
	--json "../target/doc/bevy.json" \
	--json "../target/doc/uuid.json" \
	--json "../target/doc/glam.json" \
	--config "../api_gen_config.toml" \
	> ../bevy_mod_scripting/src/api/generated.rs

make_json_files:
	rustup run nightly cargo rustdoc -p bevy_asset@0.8.0-dev  --  -Zunstable-options --output-format json && \
	rustup run nightly cargo rustdoc -p bevy_ecs@0.8.0-dev --  -Zunstable-options --output-format json && \
	rustup run nightly cargo rustdoc -p bevy_pbr@0.8.0-dev --  -Zunstable-options --output-format json && \
	rustup run nightly cargo rustdoc -p bevy_render@0.8.0-dev --  -Zunstable-options --output-format json && \
	rustup run nightly cargo rustdoc -p uuid --  -Zunstable-options --output-format json && \
	rustup run nightly cargo rustdoc -p bevy@0.8.0-dev --  -Zunstable-options --output-format json && \
	rustup run nightly cargo rustdoc -p glam --  -Zunstable-options --output-format json 