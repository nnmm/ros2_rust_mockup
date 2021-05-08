cmake_minimum_required(VERSION 3.5)
project(geometry_msgs_rs NONE)

find_package(ament_cmake)
install(
	FILES build.rs Cargo.toml DESTINATION share/${PROJECT_NAME}/rust
)
install(
  DIRECTORY src/
  DESTINATION  share/${PROJECT_NAME}/rust/src
)

install(
	DIRECTORY
  include lib
  DESTINATION .
)

ament_package()