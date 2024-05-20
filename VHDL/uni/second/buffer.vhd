-- Generatore di clock a 100 MHz
-- Stiamo simulando un oscillatore ad anello
-- * "oggetto instabile"

-- Il buffer ("buffer bit") e' un tipo di dato accessibile
-- * sia in lettura che in scrittura

entity clock_generator is
  port (clkn : buffer bit);
end clock_generator;

architecture equations of clock_generator is
begin
  clkn <= not clkn after 5 ns;
end architecture equations;

