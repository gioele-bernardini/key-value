-- Assegnazione condizionata, il buffer tri-state

library ieee;
use ieee.std_logic_1164.all;

entity three_state is
  port (
    a, enable : in std_logic;
    y : out std_logic;
  );
end entity three_state;

architecture when_else of three_state is
begin
  -- y = enable == 1 ? a : Z;
  y <= a when enable = '1' else 'Z';
end architecture when_else;

