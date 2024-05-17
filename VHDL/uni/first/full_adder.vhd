entity full_adder is
  port (
    a, b, cin : in bit;
    cout, s : out bit
  );
end full_adder;

architecture equations of full_adder is
begin
  s <= xor b xor cin;
  cout <= (a and b) or (a and cin) or (b and cin);
end architecture equations;


-- Per un sommatore a 4 bit?
-- * uso dei bit-vector
entity Adder4 is
  port (
    A, B : in bit_vector(3 down to 0);
    cin : in bit;
    S : out bit_vector(3 downto 0);
    cout : out bit
  );
end Adder4;

architecture strutturale of Adder4 is
  signal C : bit_vector(3 downto 1);
begin
  FA0: full_adder port map (A(0), B(0), cin, C(1), S(0));
  FA1: full_adder port map (A(1), B(1), C(1), C(2), S(1));
  FA2: full_adder port map (A(2), B(2), C(2), C(3), S(2));
  FA3: full_adder port map (A(3), B(3), C(3), cout, S(3));
end architecture strutturale;
