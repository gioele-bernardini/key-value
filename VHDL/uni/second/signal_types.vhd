-- Modo del segnale

entity full_adder is
  port (a, v, cin : in bit; s, cout : out bit);
end full_adder;

-- I segnali di ingresso possono solo essere letti
-- * Essere quindi a DESTRA del segno di assegnamento (<=)

-- I segnali di uscita possono solo essere scritti
-- * Essere quindi a SINISTRA del segno di assegnamento (<=)

-- L'ossido di silicio (componente CMOS) e' molto sottile
-- se colleghiamo quindi due uscite insieme per avere
-- * un super corto-circuito
-- * sto collegando potenzialmente Vdd (1) a una massa (0)

architecture equations of full_adder is begin
  s <= a xor b xor cin;
  cout <= (a and b) or (a and cin) or (b and cin);

  a <= b xnor cin;  -- Errore (ingresso a sinistra)
  s <= a and b; -- Errore (nuovo assegnamento -> conflitto)
  cout <= a or s; -- Errore (uscita a destra)
end architecture equations;

