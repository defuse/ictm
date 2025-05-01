// Sidebar & filtering logic from prototype
function checkedValues(selector) {
  return Array.from(document.querySelectorAll(selector + ':checked')).map(el => el.value);
}

function updateView() {
  const selAdvs = checkedValues('.filter-adv');
  const selSces = checkedValues('.filter-sce');

  document.getElementById('sel-advs').textContent = selAdvs.map(id =>
    document.querySelector(`.filter-adv[value="${id}"]`).parentNode.textContent.trim()
  ).join(', ');
  document.getElementById('sel-sces').textContent = selSces.map(id =>
    document.querySelector(`.filter-sce[value="${id}"]`).parentNode.textContent.trim()
  ).join(', ');

  let invCount = 0;
  document.querySelectorAll('.invariant').forEach(div => {
    const advs = div.dataset.adversaries.split(' ');
    const sces = div.dataset.scenarios.split(' ');
    const ok = selAdvs.some(a => advs.includes(a)) && selSces.some(s => sces.includes(s));
    div.classList.toggle('hidden', !ok);
    if (ok) invCount++;
  });
  document.getElementById('count-inv').textContent = invCount;

  let weakCount = 0;
  document.querySelectorAll('.weakness').forEach(div => {
    const advs = div.dataset.adversaries.split(' ');
    const sces = div.dataset.scenarios.split(' ');
    const ok = selAdvs.some(a => advs.includes(a)) && selSces.some(s => sces.includes(s));
    div.classList.toggle('hidden', !ok);
    if (ok) weakCount++;
  });
  document.getElementById('count-weak').textContent = weakCount;
}

// Wait until DOM is loaded to wire listeners and run
document.addEventListener('DOMContentLoaded', () => {
  // Wire listeners
  document.querySelectorAll('.filter-adv, .filter-sce').forEach(el =>
    el.addEventListener('change', updateView)
  );
  updateView();
});
