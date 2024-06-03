-- Latch di tipo D
entity latch is
  port (
    d, c : in std_logic;
    q : buffer std_logic
  );
end entity latch;

architecture equations of latch is
begin
  -- d quanto il clock e' attivo. altrimento q rimane q!
  q <= d when c = '1' else q;
end architecture equations;


-- Flip Flop (Edge Triggered da 0 a 1 del clock)
entity flip_flop is
  port (
    d, c : in std_logic;
    q : buffer std_logic
  );
end entity flip_flop;


architecture equations of flip_flop is
begin
  -- Alternativa >>> and not c'stable
  q <= d when c = '1' and c'event else q;
end architecture equations;


-- Flip Flop di tipo D edge triggered
