all:
	rustc src/main.rs -L rust-sdl2/ -o application

opengles:
	cd rust-opengles; make; cd ..
sdl:
	cd rust-sdl2; make; cd ..

libs: | opengles sdl

clean:
	rm application
