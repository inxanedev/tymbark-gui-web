cargo build --release
rmdir /S /Q target\WIN_BUILD
mkdir target\WIN_BUILD
copy target\release\tymbark-gui.exe target\WIN_BUILD\tymbark.exe
copy food.txt target\WIN_BUILD\food.txt

cd target\WIN_BUILD
powershell Compress-Archive . tymbark.zip