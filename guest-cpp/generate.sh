#!/bin/sh
# a bit complicated but works for now
wit-bindgen cpp-host ../wit/simple.wit --guest-header
wit-bindgen cpp ../wit/simple.wit 
