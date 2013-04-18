all:
	rust build application.rc -L rust-sdl/ -L glcore-rs/lib/ -o application

opengles:
	cd rust-opengles; make; cd ..
sdl:
	cd rust-sdl; make; cd ..

libs: | opengles sdl

clean:
	rm application
