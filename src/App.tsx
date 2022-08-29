import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Footer, Header } from "./components";
import { Error, Home, Login } from "./pages";

function App() {
  return (
    <div className="App">
      <Header />
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/login" element={<Login />} />
          <Route path="*" element={<Error code={404} />} />
        </Routes>
      </BrowserRouter>
      <Footer />
    </div>
  );
}

export default App;
