-- Porta and a 2 ingressi

-- Definisco l'entita e poi la implemento sotto
entity And2 is  
  port (x, y : in bit; z : out bit);
end entity And2;

architecture ex1 of And2 is -- Chiamo 'ex1' l'implementazione
begin
  z <= x and y; -- Assegnazione a 'z'
end architecture ex1;



-- Testbench per la And2
entity TestAnd2 is
end entity TestAnd2;

architecture simple of TestAnd2 is
  -- Segnali interni di interconnessione
  signal a, b, c : bit;
begin
  -- Istanza modulo da testare
  g1: entity And2 port map (x => a, y => b, z => c);
  -- Definisco ora gli stimoli
  -- Termino all'eccesso tutto per casini (300 al posto di 200)
  a <= '0', '1' after 100 ns, '0' after 300 ns;
  b <= '0', '1' after 150 ns, '0' after 300 ns;
end architecture simple;

-- Come effettuo quindi questa simulazione?
-- * ghdl compile And2.vhd TestAnd2.vhd -r TestAnd2
-- * per compilare e testare contemporaneamente!

-- Come MOSTRO il test? => flag --disp-time --trace-signals
-- * --vcd=test.vcd (per creare file con forme d'onda)

-- gtkwave <nome_file> per quindi visualizzare
-- * queste onde onda renderizzate prima