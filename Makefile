all:
	rust build application.rc -L rust-sdl/ -L rust-opengles/ -o application
clean:
	rm application
