using System.Collections.Generic;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using RestApi.Domain;
using RestApi.Repositories;

namespace RestApi.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class ValuesController : Controller
    {
        private readonly IValueRepository _valueRepository;

        public ValuesController(IValueRepository valueRepository)
        {
            _valueRepository = valueRepository;
        }
        
        [HttpGet("{key}")]
        public async Task<ActionResult<string>> Get(string key)
        {
            try
            {
                return Ok(await _valueRepository.GetAsync(key));
            }
            catch (KeyNotFoundException)
            {
                return NotFound();
            }
        }

        [HttpPut]
        public async Task<IActionResult> Put([FromBody] KeyValue value)
        {
            await _valueRepository.PutAsync(value);
            return Ok();
        }
    }
}