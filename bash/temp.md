
__Una serie di istruzioni da usare in Vim :)__Advertisement :)__

sed -i '/^[[:space:]]*\/\/.*$/d' name_file.txt
sed -i '/^[[:space:]]*#.*$/d' name_file.txt

:%s/\b\(i\)\b/indice/g
:1,2s/ciaone/buongiorno/g
:1,2s/$/ testo_da_aggiungere/
# ricorda il regex come eccezione!
# esempio => :1,2s/nome.*/nuovo_nome/

per copiare una parola viw => y => vado su nuova parole => viw => p
# Questo e' per vim invece :)

cat file_name | sed -n '4p' | xsel -b
# questo e' per selezionare una riga di un file e copiarla :)

sed -i '1i\Nuova riga all'inizio' file_name
sed -i '$a\Nuova riga alla fine' file_name

