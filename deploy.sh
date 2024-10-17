#!/bin/bash
set -e

fly deploy -a product-api --dockerfile ./products/Dockerfile
fly deploy -a employees-api --dockerfile ./employees/Dockerfile
fly deploy -a family-api --dockerfile  ./family/Dockerfile
fly deploy -a hobbies-api --dockerfile  ./hobbies/Dockerfile
# fly deploy -a availability-api --dockerf ./availability/Dockerfileile
fly deploy -a mood-api --dockerfile  ./mood/Dockerfile
