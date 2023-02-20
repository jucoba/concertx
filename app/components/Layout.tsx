import React from 'react';
import Header from './header/Header';
import Footer from './footer/Footer';

type LayoutProps = {
  children: React.ReactNode;
  title?: string;
};

const Layout: React.FC<LayoutProps> = ({ children, title }) => {
  return (
    <section className="min-h-screen flex flex-col">
      <Header title={title} />
      <main className="flex-grow">
        {children}
      </main>
      <Footer />
    </section>
  );
};

export default Layout;