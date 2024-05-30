import { useState, useRef, useEffect } from "react";
import "./App.css";

const Row = ({ dpi, first_name }) => {
  return (
    <div>
      {dpi}
      {first_name}
    </div>
  );
};

function Modal({ openModal, closeModal, children }) {
  const ref = useRef();

  useEffect(() => {
    if (openModal) {
      ref.current?.showModal();
    } else {
      ref.current?.close();
    }
  }, [openModal]);

  return (
    <dialog ref={ref} onCancel={closeModal}>
      <div>
        {children}
        <button onClick={closeModal} className="btn" type="button">
          Cerrar
        </button>
      </div>
    </dialog>
  );
}

const Button = ({ text, onClick }) => {
  return (
    <button className="btn" onClick={onClick}>
      {text}
    </button>
  );
};

function App() {
  const [database, setDatabase] = useState(null);
  const [file, setFile] = useState(null);
  const [result, setResult] = useState([]);
  const [modal, setModal] = useState(false);

  const handleFileChange = (event) => {
    setFile(event.target.files[0]);
  };

  const handleDatabaseChange = (event) => {
    setDatabase(event.target.files[0]);
  };

  const handleSubmit = async (event) => {
    event.preventDefault();
    if (!file) {
      alert("Please select the auction!");
      return;
    }
    if (!database) {
      alert("Please select the customers database!");
      return;
    }

    const formData = new FormData();
    formData.append("file", file);
    formData.append("database", database);
    try {
      const response = await fetch("http://localhost:8000/upload", {
        method: "POST",
        body: formData,
      });

      if (response.ok) {
        let data = await response.json();
        data = data.map((current) => JSON.parse(current));
        alert("File uploaded successfully!");
        setResult(data);
      } else {
        alert("File upload failed!");
        setResult(null);
      }
    } catch (error) {
      alert(`Error uploading file: ${error.message}`);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>InMuebles GT: Interfaz de busqueda</h1>
        <Button text="Salir" onClick={null} />
      </header>
      <main className="content">
        <div className="blocker">Ingrese datos</div>
        {result.map((current) => (
          <Row
            key={current.dpi}
            dpi={current.dpi}
            first_name={current.first_name}
          />
        ))}
      </main>
      <div className="instructions">
        <p className="instructions-title">Instrucciones</p>
        <ul className="instructions-list">
          <li>
            Ingresar los Json´s con el botón inferior, ingresar clientes y
            postores.
          </li>
          <li>Los clientes ganadores se mostraran en la pantalla</li>
          <li>Ingresar solamente Json´s validos</li>
        </ul>
      </div>
      <button onClick={() => setModal(true)} className="btn" type="button">
        Ingresar Datos
      </button>
      <div>
        <Modal openModal={modal} closeModal={() => setModal(false)}>
          <form onSubmit={handleSubmit} className="main-form">
            <div className="input-container">
              <label htmlFor="databse">Clientes</label>
              <input
                type="file"
                onChange={handleDatabaseChange}
                id="database"
                name="database"
              />
              <label htmlFor="file">Subasta</label>
              <input
                type="file"
                onChange={handleFileChange}
                id="file"
                name="file"
              />
            </div>
            <div className="upload">
              <button type="submit" className="btn">
                Upload
              </button>
            </div>
          </form>
        </Modal>
      </div>
    </div>
  );
}

export default App;
