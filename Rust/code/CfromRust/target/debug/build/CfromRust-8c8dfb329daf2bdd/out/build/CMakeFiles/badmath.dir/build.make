# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.22

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = "/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/libbadmath"

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = "/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/target/debug/build/CfromRust-8c8dfb329daf2bdd/out/build"

# Include any dependencies generated for this target.
include CMakeFiles/badmath.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/badmath.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/badmath.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/badmath.dir/flags.make

CMakeFiles/badmath.dir/badmath.c.o: CMakeFiles/badmath.dir/flags.make
CMakeFiles/badmath.dir/badmath.c.o: /mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year\ 4\ Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/libbadmath/badmath.c
CMakeFiles/badmath.dir/badmath.c.o: CMakeFiles/badmath.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir="/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/target/debug/build/CfromRust-8c8dfb329daf2bdd/out/build/CMakeFiles" --progress-num=$(CMAKE_PROGRESS_1) "Building C object CMakeFiles/badmath.dir/badmath.c.o"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -MD -MT CMakeFiles/badmath.dir/badmath.c.o -MF CMakeFiles/badmath.dir/badmath.c.o.d -o CMakeFiles/badmath.dir/badmath.c.o -c "/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/libbadmath/badmath.c"

CMakeFiles/badmath.dir/badmath.c.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing C source to CMakeFiles/badmath.dir/badmath.c.i"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -E "/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/libbadmath/badmath.c" > CMakeFiles/badmath.dir/badmath.c.i

CMakeFiles/badmath.dir/badmath.c.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling C source to assembly CMakeFiles/badmath.dir/badmath.c.s"
	/usr/bin/cc $(C_DEFINES) $(C_INCLUDES) $(C_FLAGS) -S "/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/libbadmath/badmath.c" -o CMakeFiles/badmath.dir/badmath.c.s

# Object files for target badmath
badmath_OBJECTS = \
"CMakeFiles/badmath.dir/badmath.c.o"

# External object files for target badmath
badmath_EXTERNAL_OBJECTS =

libbadmath.a: CMakeFiles/badmath.dir/badmath.c.o
libbadmath.a: CMakeFiles/badmath.dir/build.make
libbadmath.a: CMakeFiles/badmath.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir="/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/target/debug/build/CfromRust-8c8dfb329daf2bdd/out/build/CMakeFiles" --progress-num=$(CMAKE_PROGRESS_2) "Linking C static library libbadmath.a"
	$(CMAKE_COMMAND) -P CMakeFiles/badmath.dir/cmake_clean_target.cmake
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/badmath.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/badmath.dir/build: libbadmath.a
.PHONY : CMakeFiles/badmath.dir/build

CMakeFiles/badmath.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/badmath.dir/cmake_clean.cmake
.PHONY : CMakeFiles/badmath.dir/clean

CMakeFiles/badmath.dir/depend:
	cd "/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/target/debug/build/CfromRust-8c8dfb329daf2bdd/out/build" && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" "/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/libbadmath" "/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/libbadmath" "/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/target/debug/build/CfromRust-8c8dfb329daf2bdd/out/build" "/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/target/debug/build/CfromRust-8c8dfb329daf2bdd/out/build" "/mnt/15f210d7-aa4f-4ea8-a4b3-2bd52aeba8fc/Programming/Uni/Year 4 Hons/Research-Improving-Device-Drivers/Rust/code/CfromRust/target/debug/build/CfromRust-8c8dfb329daf2bdd/out/build/CMakeFiles/badmath.dir/DependInfo.cmake" --color=$(COLOR)
.PHONY : CMakeFiles/badmath.dir/depend

