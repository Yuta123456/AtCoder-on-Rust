@echo off

for /d %%d in (*) do (
  echo filename = %%d
  cd ./%%d
  rm -rf .git/
  cd ..
  
)