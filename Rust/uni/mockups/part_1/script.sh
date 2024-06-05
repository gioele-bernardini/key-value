#!/bin/bash

# Ottieni tutti i file che seguono il pattern 'file<number>'
for file in mockup[0-9]*; do
  # Estrai il numero dal nome del file
  num=$(echo "$file" | grep -o '[0-9]\+')
  
  # Calcola il nuovo numero incrementato
  new_num=$((num + 1))
  
  # Crea il nuovo nome del file con un prefisso temporaneo
  new_file="temp_$(echo "$file" | sed "s/$num$/$new_num/")"
  
  # Rinomina il file al nome temporaneo
  mv "$file" "$new_file"
done

# Rimuovi il prefisso temporaneo dai file rinominati
for temp_file in temp_file[0-9]*; do
  final_file=$(echo "$temp_file" | sed 's/^temp_//')
  mv "$temp_file" "$final_file"
done

