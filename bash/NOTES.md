sed -i '/^[[:space:]]*\/\/.*$/d' name_file.txt
sed -i '/^[[:space:]]*#.*$/d' name_file.txt

:%s/\b\(i\)\b/indice/g
# Questo e' per vim invece :)
