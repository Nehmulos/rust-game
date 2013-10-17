all:
	rustc build game.rc -L rust-sdl2/ -L rust-opengles/ -o application

opengles:
	cd rust-opengles; make; cd ..
sdl:
	cd rust-sdl2; make; cd ..

libs: | opengles sdl

clean:
	rm application
