#!/bin/bash

echo
echo "dictate"
echo "by ShadowNetter"
echo
echo "cloning into repo..."
git clone https://github.com/ShadowNetter-Official/dictate
cd dictate
echo "done"
echo "installing..."
chmod +x dictate
cp dictate ~/.cargo/bin/
echo "done"
echo
echo "to uninstall do: "
echo "rm ~/.cargo/bin/dictate"
echo
