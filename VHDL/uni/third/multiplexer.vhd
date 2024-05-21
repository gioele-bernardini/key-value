-- Multiplexer, assegnazione selezionata

library ieee;
use ieee.std_logic_1164.all;

entity multiplexer is
  port (
    a, b, c : in std_logic;
    sa, sb, sc : in std_logic;
    y : out std_logic
  );
end entity multiplexer;

architecture with_select of multiplexer is
  signal selez : std_logic_vector (2 downto 0);
begin
  selez <= (sa, sb, sc);
  with selez select
  y <= when "100",
  b when "010",
  c when  "001",
  '0' when others;
end architecture with_select;
  
