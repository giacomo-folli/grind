## Problemi prioritari (alta priorità)

### 1) Errori di salvataggio ignorati
In più funzioni viene usato `let _ = storage::save(&tasks);`.

**Rischio**: se il salvataggio fallisce, l'utente non riceve errore e crede che l'operazione sia andata a buon fine.

**Miglioria**:
- Propagare sempre l'errore con `?`.
- Aggiungere contesto all'errore (`anyhow::Context`) nei layer esterni.

### 2) `edit` non aggiorna sia titolo che descrizione nello stesso comando
La funzione `edit_task` interrompe il flusso (`break`) subito dopo aver aggiornato uno dei due campi.

**Rischio**: comportamento inatteso se l'utente passa `--title` e `--description` insieme.

**Miglioria**:
- Applicare entrambi gli update nello stesso match.
- Chiamare `update_time()` una sola volta se almeno un campo cambia.

### 3) Operazioni mute quando ID non trovato
`status`, `edit` e `delete` terminano con successo anche se non trovano il task.

**Rischio**: UX ambigua, debugging difficile.

**Miglioria**:
- Introdurre un errore di dominio (`TaskNotFound`).
- Stampare feedback esplicito all'utente.

### 4) Possibile panic su slicing dell'ID
Nel rendering della lista viene usato `&task.id[..8]`.

**Rischio**: panic se il file TOML contiene ID più corto (es. edit manuale).

**Miglioria**:
- Usare `task.id.get(..8).unwrap_or(&task.id)` oppure un approccio Unicode-safe con `chars().take(8)`.

---

## Architettura e design

### 5) Logica business troppo concentrata in `main.rs`
Attualmente `main.rs` contiene parsing CLI, orchestrazione e logica CRUD.

**Miglioria**:
- Introdurre un service layer (`app.rs` / `service.rs`) con API testabili.
- Lasciare a `main.rs` solo parsing argomenti e dispatch.

### 6) Incoerenze tra README e implementazione
La documentazione non è perfettamente allineata al modello dati serializzato.

**Miglioria**:
- Allineare naming e schema (`state/status`, chiavi TOML, formato esempi).
- Allineare anche sezione dipendenze/ID generator.

### 7) Gestione del tempo migliorabile
I timestamp sono `String` RFC2822.

**Limite**: meno robustezza a compile-time e parsing ricorrente in fase di output.

**Miglioria**:
- Valutare `DateTime<Utc>` nel modello.
- Usare RFC3339 e gestire esplicitamente date future nel formato relativo.

---

## Metodi e robustezza Rust

### 8) Tipi di errore non uniformi
Nel codice si alternano `anyhow::Result` e `Result<_, StateError>` senza una separazione netta tra layer.

**Miglioria**:
- Definire una policy: errori tipizzati nel core, `anyhow` al boundary CLI.

### 9) Default descrizione in `Task::new`
La descrizione viene inizializzata con `Some(String::new())`.

**Miglioria**:
- Preferire `None` come assenza di valore.
- Mostrare la descrizione in lista solo se presente e non vuota.

### 10) `init` distruttivo su file esistente
`File::create` tronca il file se già presente.

**Miglioria**:
- Rendere `init` idempotente o esplicitare il comportamento con `--force`.

---

## Roadmap consigliata

- [ ] Correggere subito gestione errori di `save`.
- [ ] Introdurre `TaskNotFound` e feedback consistente.
- [ ] Sistemare `edit` per supportare update multiplo.
- [ ] Rimuovere rischio panic nello slicing ID.
- [ ] Estrarre service layer e aggiungere test unitari sul core.
- [ ] Allineare README e modello dati reale.
