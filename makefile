lambdas_dir=src/bin
zipped_dir=zipped

build-musl=cargo build --release --target x86_64-unknown-linux-musl


define zip =
zip ./${zipped_dir}/$(1).zip target/x86_64-unknown-linux-musl/release/$(1)
endef

LAMBDAS = \
		first second

release:
	${build-musl}
	@$(MAKE) zip_all

zip_all:
	@$(MAKE) $(addprefix zip-,$(LAMBDAS))

zip-%:
	$(call zip,$*)

$(LAMBDAS):
	(${build-musl})
	@$(MAKE) zip-$(@)

.PHONY: clean
clean:
	cargo clean
	-rm -f ./zipped/*.zip

