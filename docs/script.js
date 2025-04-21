document.addEventListener('DOMContentLoaded', () => {
  const toggleBtn = document.getElementById('toggle-btn');
  const sidebar = document.getElementById('sidebar');

  toggleBtn.addEventListener('click', () => {
    sidebar.classList.toggle('open');
  });

  // Highlight active link
  const links = sidebar.querySelectorAll('a');
  links.forEach(link => {
    if (link.getAttribute('href') === window.location.pathname.split('/').pop()) {
      link.classList.add('active');
    }
  });
});
