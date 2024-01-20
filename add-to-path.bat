@echo off
REM Change the path below to the path of your built executable
set RELEASE_DIR=%~dp0\target\release
REM Add the directory to the PATH environment variable
setx Path "%Path%;%RELEASE_DIR%"