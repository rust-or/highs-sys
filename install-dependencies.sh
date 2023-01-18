set -x
if test -x "$(which apt-get)"; then
  sudo apt-get install libstdc++6 libgomp1 cmake
elif test -x "$(which dnf)"; then
  sudo dnf install libstdc++ libgomp cmake
elif test -x "$(which brew)"; then
  brew install libomp cmake
  brew link --force libomp
else
  echo "system not supported"
  exit 1
fi
