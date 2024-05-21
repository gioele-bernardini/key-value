-- Prevede molte alternative e puo' essere laborioso
-- * per questo e' previsto uno stile di
-- * SPECIFICA COMPORTAMENTALE
-- Sono parti di codice in sequenza, invece che in concorrenza

-- Sintassi di un processo
-- Il processo viene eseguito ogni volta che
-- * uno dei segnali nella list cambia valore!
architecture comportamentale of mio_modulo is
begin
  label: process (sensitivity-list) is
    dichiarazioni di tipi, costanti, segnali, variabili;

  begin
    statement-sequenziale;
    statement_sequenziale;
  end process;
end architecture comportamentale;

