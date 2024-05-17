-- Uso di espressioni multiple con ritardi

entity comb_function is
  port (a, b, c : in bit; z : out bit);
end entity comb_function;

architecture expression of comb_function is
  signal d : bit;
begin
  -- L'ordine delle equazioni non e' influente!
  -- I ritardi saranno calcolati e conosciuti da Vivado
  -- Di solito non si mettono ritardi in questo file
  -- Nella sintesi li metteremo via Vivado in automatico
  -- Queste sono quindi tempistiche arbitrarie
  z <= d or c after 5 ns;
  d <= a and b after 4 ns;
end architecture expression;

-- Testbench
entity TestCombFunction is
end entity TestCombFunction;

architecture simple of TestCombFunction is
  -- Segnali interni di interconnessione
  signal a, b, c, z : bit;
begin
  -- Istanza modulo da testare
  g1: entity work.comb_function port map (
    -- i primi valori sono dell'entita', i secondi di questa architettura
    a => a,
    b => b,
    c => c,
  );
  -- Definizione degli stimoli
  a <= '0', '1' after 100 ns, '0' after 200 ns, '0' after 300 ns;
  b <= '0', '1' after 150 ns, '0' after 250 ns; '0' after 300 ns;
  c <= '1', '0' after 100 ns;
end architecture TestCombFunction;

