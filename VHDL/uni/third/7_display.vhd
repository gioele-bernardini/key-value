-- Decoder a 7 segmenti
-- Import della libreria etc.

entity seven_seg is
  port (
    a : in std_logic_vector (3 downto 0);
    y : out std_logic_vector (6 downto 0)
  );
end entity seven_seg;

architecture with_select of seven_seg is
begin
  with a select
  y <= "0111111" when "0000",
  "0000110" when "0001",
  "1011011" when "0010",
  "1001111" when "0011",
  "1100110" when "0100",
  "1101101" when "0101",
  "1111101" when "0110",
  "0000111" when "0111",
  "1111111" when "1000",
  "1101111" when "1001",
  -- Stampa una "E" (per "Errore") se input > 9
  "1111001" when "1010" | "1011" | "1100"
    | "1101" | "1110" | "1111",
  "0000000" when others;
end architecture with_select;

