all:
	rust build application.rc -L rust-sdl/ -L rust-opengles/ -o application

opengles:
	cd rust-opengles; make; cd ..
sdl:
	cd rust-sdl; make; cd ..

libs: | opengles sdl

clean:
	rm application
