@echo off

set LIB=unitytts
set OPT=--release
set DST=.\package\Runtime\Plugin

cargo build %OPT%
copy /Y target\release\%LIB%.dll %DST%\Windows