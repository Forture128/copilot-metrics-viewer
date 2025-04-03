import { Chart, registerables } from 'chart.js'
import 'chartjs-adapter-date-fns'

// Register all Chart.js components (bar, line, etc.)
Chart.register(...registerables)

export default Chart
