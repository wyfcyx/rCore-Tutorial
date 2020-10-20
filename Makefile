run:
	@make -C user sfsimg
	@make -C os run

clean:
	@make -C user clean
	@make -C os clean

fmt:
	@cd os && cargo fmt
	@cd os/src/algorithm && cargo fmt
	@cd user && cargo fmt
