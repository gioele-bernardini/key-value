sed -i '/^[[:space:]]*\/\/.*$/d' name_file.txt
sed -i '/^[[:space:]]*#.*$/d' name_file.txt

:%s/\b\(i\)\b/indice/g
# Questo e' per vim invece :)

cat file_name | sed -n '4p' | xsel -b
# questo e' per selezionare una riga di un file e copiarla :)

sed -i '1i\Nuova riga all'inizio' nome_del_tuo_file.txt
sed -i '$a\Nuova riga alla fine' nome_del_tuo_file.txt

