set -x
if test -x "$(which brew)"; then
  brew install libomp
elif test -x "$(which apt-get)"; then
  sudo apt-get install libstdc++6 libgomp1
else
  echo "system not supported"
  exit 1
fi
