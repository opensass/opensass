run:
	dx serve
build:
	dx build --release
ssg:
	IP=0.0.0.0 PORT=3000 dx bundle --fullstack --ssg --release
