-- Uso di espressioni multiple

entity comb_function is
  port (a, b, c : in bit; z : out bit);
end entity comb_function;

architecture expression of comb_function is
  -- 'd' e' un segnale interno (intermedio all'architettura)
  -- * non e' quindi da mettere nella def. dell'entita'!
  -- * serve solo a noi per comodita' come punto intermedio
  signal d : bit;
begin
  -- z <= (a and b) or c;
  d <= a and b;
  z <= d or c;
end architecture expression;

