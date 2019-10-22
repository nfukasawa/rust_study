import React from 'react';
import logo from './logo.svg';
import './App.css';

const App: React.FC = () => {
  const [width, height] = [100, 100];

  React.useEffect(() => {
    (async () => {
      const lifegame = await import('lifegame/lifegame');
      const { LifeGame } = lifegame;

      const game = LifeGame.new(width, height);
      const buf = new Uint8Array(width * height);
      game.fill_cells(buf);
    })();
  }, [width, height]);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
