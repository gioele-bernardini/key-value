-- Segnali sia di ingresso che di uscita
-- Dal punto di vista del componente sono come i buffer
-- * ma possono avere un driver anche dall'esterno, i buffer NO

-- Possono inoltre apparire A SINISTRA di piu' espressioni
-- * ma occorre risolvere eventuali conflitti

entity bus_interface is
  port (
    en, d_in : in bit;
    d_out : out bit
    bus : inout bit
  );
end entity bus_interface;

-- Sono di solito usati per rappresentare buffer tri-state
