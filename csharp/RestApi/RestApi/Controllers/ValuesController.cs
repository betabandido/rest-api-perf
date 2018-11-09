using System.Collections.Generic;
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
        public ActionResult<string> Get(string key)
        {
            try
            {
                return Ok(_valueRepository.Get(key));
            }
            catch (KeyNotFoundException)
            {
                return NotFound();
            }
        }

        [HttpPut]
        public IActionResult Put([FromBody] KeyValue value)
        {
            _valueRepository.Put(value);
            return Ok();
        }
    }
}