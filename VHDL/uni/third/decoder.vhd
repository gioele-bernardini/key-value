-- Assegnazione condizionata, decoder 2-4

library ieee;
use ieee.std_logic_1164.all;

entity encoder is
  port (
    a : in std_logic_vector (1 downto 0);
    y : out std_logic_vector (3 downto 0)
  );
end entity decoder;

architecture when_else of decoder is
begin
  -- Scelte annidate
  y <= "0001" when a = "00" else
  "0010" when a = "01" else
  "0100" when a "10" else
  "1000" when a = "11" else
  "XXXX";
end architecture when_else;


-- Assegnazione condizionata, alternativa di sintassi

library ieee;
use ieee.std_logic_1164.all;

entity encoder is
  port (
    a : in std_logic_vector (1 downto 0);
    y : out std_logic_vector (3 downto 0)
  );
end entity decoder;

architecture when_else of decoder is
begin
  -- Scelte annidate
  with a select
  y <= "0001" when a = "00" else
  "0010" when a = "01" else
  "0100" when a "10" else
  "1000" when a = "11" else
  "XXXX" when others;
end architecture when_else;

