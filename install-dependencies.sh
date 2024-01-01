set -x
if test -x "$(which apt-get)"; then
  sudo apt-get install libstdc++6 cmake
elif test -x "$(which dnf)"; then
  sudo dnf install libstdc++ cmake
elif test -x "$(which brew)"; then
  echo "Nothing to instal on MacOS"
else
  echo "system not supported"
  exit 1
fi
