cmake_minimum_required(VERSION 3.10)

# Project name
project(NeuralNetworkProject)

# Set C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add include directories
include_directories(include)

# Add source files
set(SOURCES
    Layer.cpp
    main.cpp
    Matrix.cpp
    Neuron.cpp
)

# Create executable
add_executable(NeuralNetwork ${SOURCES})

