all:
	rustc src/main.rs -L nphysics/lib -L nphysics/examples/lib -L nphysics/ncollide/nalgebra/lib/ -L nphysics/ncollide/lib -o game

rsfml:
	cd nphysics; make examples_deps; cd ..;

nphysics:
	cd nphysics; make; cd ..;

libs: | nphysics csfml

clean:
	rm application
